
export const ENTRY_EXTS = ['js', 'ts', 'tsx', 'jsx', 'vue']


const userConfig = getUserConfig()
const buildIgnore = userConfig?.build?.ignore || []

export function hasDefaultExport(code: string) {
  return code.includes('export default') || code.includes('export { default }')
}

const EXCLUDES = ['.DS_Store',...buildIgnore]
  const dirs = readdirSync(SRC_DIR)

  return dirs
    .filter((dir) => !EXCLUDES.includes(dir))
    .filter((dir) =>
      ENTRY_EXTS.some((ext) => {
        const path = join(SRC_DIR, dir, `index.${ext}`)
        if (existsSync(path)) {
          return hasDefaultExport(readFileSync(path, 'utf-8'))
        }

        return false
      })
    )
