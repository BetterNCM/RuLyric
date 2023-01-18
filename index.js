plugin.onLoad(() => {
    betterncm_native.native_plugin.call('rulyrics.init_lyrics_app', [])
    let cnt = 3;
    let mode = 1;

    let lastUpd = 0;
    let lastStr = "";

    let mlyricUpdTime = 0

    betterncm.utils.waitForElement("#x-g-mn .m-lyric").then((lyric) => {
        new MutationObserver(mutations => {
           
            const lyrics = [...document.querySelectorAll('#x-g-mn .m-lyric>p>span'), ...document.querySelectorAll('#x-g-mn .m-lyric>p')].map(v => v.innerText);
            if (lastStr === lyrics[0]) return;
            mlyricUpdTime = new Date().getTime();

            if (mode !== 1 && (new Date().getTime() - lastUpd) < 300) return;


            setTimeout(() => {
                if (mode !== 1 && (new Date().getTime() - lastUpd) < 300) return;

                betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
                    [
                        lyrics[0].split(" ").map(v => ([`${v} `, 200])), cnt++
                    ],
                    lyrics[1] || ""
                ])

                betterncm_native.native_plugin.call('rulyrics.seek', [300, false]);
            }, 300);

        }).observe(lyric, { childList: true, subtree: true });
    })



    const amllObserver = new MutationObserver(mutations => {
        try {
            if (document.querySelector(".am-lyric-line-selected .am-lyric-line-dynamic") == null
                || new Date().getTime() - lastUpd < 100) return;
            console.log("AMLL Upd")
            lastUpd = new Date().getTime();

            const lrc = [...document.querySelector(".am-lyric-line-selected .am-lyric-line-dynamic").children].map(v => [
                v.firstChild.innerText, parseInt(v.firstChild.style.animationDuration)
            ]);
            const sLrc = lrc.map(v => v[0]).join('')
            if (lastStr === sLrc) return;
            lastStr = sLrc

            betterncm_native.native_plugin.call('rulyrics.update_lyrics', [
                [
                    lrc, cnt++
                ],
                document.querySelector(".am-lyric-line-selected .am-lyric-line-translated")?.innerText || ""
            ])
            if (mlyricUpdTime !== 0) {
                console.log("Seeked",new Date().getTime() - mlyricUpdTime);
                betterncm_native.native_plugin.call('rulyrics.seek', [new Date().getTime() - mlyricUpdTime + 10 /*for api delay*/, false]);
                mlyricUpdTime = 0;
            }
            

            mode = 2
        } catch (e) {
            mode = 1
        }
    });
    window.obs = amllObserver
    betterncm.utils.waitForElement(".g-singlec-ct").then((lyric) => {
        let amv;
        setInterval(() => {
            if (amv !== document.querySelector("#applemusic-like-lyrics-view")) {
                amv = document.querySelector("#applemusic-like-lyrics-view");
                amllObserver.observe(amv, { attributes: true, childList: true, subtree: true });
            }

        }, 1000)
    })
    setTimeout(()=>{
        betterncm_native.native_plugin.call('rulyrics.embed_into_taskbar', [])
    },100);
});