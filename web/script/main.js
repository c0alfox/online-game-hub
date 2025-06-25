const heartSize = 48; // px
const animationDuration = 800; // ms

const ws = new WebSocket('/ws/hearts');
ws.onclose = () => {
    console.log("Connection closed");
}

/**
 * 
 * @param {MessageEvent} e 
 */
ws.onmessage = (e) => {
    const json = JSON.parse(e.data);
    const x = json.x;
    const y = json.y;

    if (x == undefined || y == undefined) {
        x = window.innerWidth / 2;
        y = window.innerHeight / 2;
    }

    spawnHeart(x, y);
}

const spawnHeart = (x, y) => {
    let elem = document.createElement('div');
    elem.classList.add('heart');
    elem.style.left = `${x - heartSize / 2}px`;
    elem.style.top = `${y - heartSize / 2}px`;

    document.body.appendChild(elem);

    setTimeout(() => {
        document.body.removeChild(elem);
    }, animationDuration);
}

document.addEventListener('click', e => {
    spawnHeart(e.clientX, e.clientY);

    const msg_body = {
        x: e.clientX,
        y: e.clientY
    };

    ws.send(JSON.stringify(msg_body));
});
