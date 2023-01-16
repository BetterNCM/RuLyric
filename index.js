plugin.onLoad(() => {
    betterncm_native.native_plugin.call('rulyrics.init_lyrics_app', [])

    setInterval(() => {
        let lyrics=new Array(...document.querySelectorAll('.j-line.z-crt *')).map(v => v.innerText);
        betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
            lyrics[0],
            lyrics[1] || ""
        ])
    }, 100)
});