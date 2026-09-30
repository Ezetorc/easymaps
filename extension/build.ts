const result = await Bun.build({
    entrypoints: [
        './src/background.ts',
        './src/adapters/minecraft-maps.adapter.ts',
    ],
    outdir: './dist',
    target: 'browser',
    format: 'esm',
    naming: '[dir]/[name].[ext]',
    splitting: false,
})

if (!result.success) {
    console.error('Build failed')

    for (const log of result.logs) {
        console.error(log)
    }

    process.exit(1)
}
