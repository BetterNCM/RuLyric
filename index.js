plugin.onLoad(() => {
    betterncm_native.native_plugin.call('rulyrics.init_lyrics_app', [])

    setInterval(() => {
        betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
            new Array(...document.querySelectorAll('.j-line.z-crt *')).map(v => v.innerText).join('\n')
        ])
    }, 100)
});