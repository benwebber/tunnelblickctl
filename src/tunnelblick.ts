// JXA Globals
declare const Application: any;
declare const Path: any;

enum RPCStatus {
  Success = 'success',
  Error = 'error',
}

interface Configuration {
  readonly name: string;
  readonly state: string;
  readonly autoconnect: string;
  readonly bytesIn: number;
  readonly bytesOut: number;
}

interface RPCResponse {
  readonly status: RPCStatus;
  readonly data: unknown;
}

class TunnelblickController {
  get app(): any {
    try {
      const app = Application('Tunnelblick');
      app.includeStandardAdditions = true;
      return app;
    } catch (err) {
      const e = err as { errorNumber: number; message: string };
      if (e.errorNumber === -2700 && e.message.match(/Application can't be found/)) {
        throw new Error('Tunnelblick is not installed');
      }
      throw err;
    }
  }

  connect(name: string): boolean {
    this.assertRunning();
    this.assertConfigurationExists(name);
    return this.app.connect(name);
  }

  connectAll(): number {
    this.assertRunning();
    return this.app.connectAll();
  }

  disconnect(name: string): boolean {
    this.assertRunning();
    this.assertConfigurationExists(name);
    return this.app.disconnect(name);
  }

  disconnectAll(): number {
    this.assertRunning();
    return this.app.disconnectAll();
  }

  getStatus(): Configuration[] {
    return this.getConfigurations();
  }

  getVersion(): string {
    return this.app.version();
  }

  install(path: string): void {
    this.assertRunning();
    const finder = Application('Finder');
    return finder.open([Path(path)], { using: this.app.pathTo() });
  }

  launch(): boolean {
    return this.app.launch();
  }

  list(): string[] {
    return this.getConfigurations().map((config) => config.name);
  }

  quit(): boolean {
    this.assertRunning();
    this.app.quit();
    return true;
  }

  private assertRunning(): void {
    if (!this.app.running()) {
      throw new Error('Tunnelblick is not running');
    }
  }

  private assertConfigurationExists(name: string): void {
    const config = this.getConfigurations().find((c) => c.name === name);
    if (!config) {
      throw new Error(`VPN '${name}' does not exist`);
    }
  }

  private getConfigurations(): Configuration[] {
    this.assertRunning();
    const length = this.app.configurations().length;
    return Array.from({ length }, (_, i) => {
      const config = this.app.configurations.at(i);
      return {
        name: config.name(),
        state: config.state(),
        autoconnect: config.autoconnect(),
        bytesIn: Number.parseInt(config.bytesin(), 10),
        bytesOut: Number.parseInt(config.bytesout(), 10),
      };
    });
  }
}

class RPC {
  constructor(readonly service: any) {}

  call(method: string, ...args: any[]): RPCResponse {
    try {
      const data = this.service[method](...args);
      return { status: RPCStatus.Success, data };
    } catch (err) {
      return { status: RPCStatus.Error, data: (err as Error).message };
    }
  }
}

const tunnelblickctl = new TunnelblickController();
const rpc = new RPC(tunnelblickctl);
