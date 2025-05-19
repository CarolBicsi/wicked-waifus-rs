setTimeout(() => {
    const UiManager_1 = require("../Ui/UiManager");
    const UE = require("ue");
    const ControllerManagerBase_1 = require("../../Core/Framework/ControllerManagerBase");
    
    const UiText = UiManager_1.UiManager.GetViewByName("UidView").GetText(0);
    UiText.SetText("原神再起不能动");
    
    // 颜色渐变实现
    let hue = 0;
    const updateInterval = 50; // 更新频率(毫秒)
    
    // 创建定时器实现颜色循环变化
    const colorTimer = setInterval(() => {
        // 通过HSV颜色空间实现平滑渐变，只改变色相(H)
        hue = (hue + 1) % 360;
        
        // 将HSV转换为RGB，饱和度和亮度保持最大
        const color = hsvToRgb(hue / 360, 1, 1);
        
        // 创建UE颜色对象并应用
        const ueColor = new UE.Color(color.r, color.g, color.b, 1);
        UiText.SetColor(ueColor);
    }, updateInterval);
    
    // HSV转RGB的辅助函数
    function hsvToRgb(h, s, v) {
        let r, g, b;
        const i = Math.floor(h * 6);
        const f = h * 6 - i;
        const p = v * (1 - s);
        const q = v * (1 - f * s);
        const t = v * (1 - (1 - f) * s);
        
        switch (i % 6) {
            case 0:
                r = v;
                g = t;
                b = p;
                break;
            case 1:
                r = q;
                g = v;
                b = p;
                break;
            case 2:
                r = p;
                g = v;
                b = t;
                break;
            case 3:
                r = p;
                g = q;
                b = v;
                break;
            case 4:
                r = t;
                g = p;
                b = v;
                break;
            case 5:
                r = v;
                g = p;
                b = q;
                break;
        }
        
        return {
            r: Math.round(r * 255),
            g: Math.round(g * 255),
            b: Math.round(b * 255)
        };
    }
}, 10000);
