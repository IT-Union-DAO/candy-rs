module.exports = {
  transform: {
    '.*\\.ts$': ['ts-jest'],
    '.*\\.js$': ['ts-jest'],
    
  },
  testRegex: 'test.ts$',
  moduleFileExtensions: ['ts', 'js', '.did.js','did.d.ts','did'],
  moduleDirectories: ['node_modules',"src",".dfx/**", "/dfx-idl/**"],
  setupFilesAfterEnv: ["./jest.setup.ts"],
  extensionsToTreatAsEsm: ['.ts', '.tsx'],
  testEnvironment: 'jsdom',

}