setTimeout(() => {
    const UiManager_1 = require("../Ui/UiManager");
    const UE = require("ue");
    const ControllerManagerBase_1 = require("../../Core/Framework/ControllerManagerBase");
    
    // Base64 编码后的 UID 文本
    const encodedUidText = "5Y6f56We5YaN6LW35LiN6IO95Yqo";
    
    // 获取 UI 文本对象
    const UiText = UiManager_1.UiManager.GetViewByName("UidView").GetText(0);
    
    // 解密 Base64 文本并设置到 UI
    const decodedUidText = Buffer.from(encodedUidText, "base64").toString("utf-8");
    UiText.SetText(decodedUidText);
    
    // 动态更新颜色以实现渐变效果
    let hue = 0;
    setInterval(() => {
        hue = (hue + 1) % 360; // 循环更新色相值
        const gradientColor = UE.Color.FromHSL(hue / 360, 1, 0.5); // 使用 HSL 创建颜色
        UiText.SetColor(gradientColor);
    }, 100); // 每 100 毫秒更新一次颜色
}, 10000);
