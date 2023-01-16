plugin.onLoad(() => {
    betterncm_native.native_plugin.call('rulyrics.init_lyrics_app', [])
    let cnt = 3;
    let mode = 1;
    betterncm.utils.waitForElement("#x-g-mn .m-lyric").then((lyric) => {
        new MutationObserver(mutations => {
            if (mode !== 1) return;

            for (const mutation of mutations) {
                const lyrics = new Array(...mutation.addedNodes).map(v => v.innerText);
                betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
                    [
                        lyrics[0].split(" ").map(v => ([`${v} `, 200])), cnt++
                    ],
                    lyrics[1] || ""
                ])
            }

        }).observe(lyric, { childList: true, subtree: true });
    })


    betterncm.utils.waitForElement("#applemusic-like-lyrics-view").then((lyric) => {
        mode = 2;

        let lastStr;

        betterncm_native.native_plugin.call('rulyrics.embed_into_taskbar', [])

        setInterval(()=>{
            try{
                const lrc=[...document.querySelector(".am-lyric-line-selected .am-lyric-line-dynamic").children].map(v => [
                    v.firstChild.innerText, parseInt(v.firstChild.style.animationDuration)
                ]);
                const sLrc=lrc.map(v=>v[0]).join('')
                if(lastStr===sLrc)return;
                lastStr=sLrc

                betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
                    [
                        lrc, cnt++
                    ],
                    document.querySelector(".am-lyric-line-selected .am-lyric-line-translated")?.innerText || ""
                ])
            }catch(e){
                console.error(e)
            }
        },10)
    })
});