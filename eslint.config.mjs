import tseslint from 'typescript-eslint';

export default tseslint.config(tseslint.configs.recommended, {
  rules: {
    '@typescript-eslint/no-explicit-any': 'off',
    '@typescript-eslint/no-unused-vars': 'off',
  },
});
