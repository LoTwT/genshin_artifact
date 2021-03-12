export default [
    {
        time: "2021/2/20",
        version: "2.0.1",
        changes: [
            "能够计入乐团4、角斗士4的效果",
            "优化了UI",
            "增加甘雨的专属目标函数",
        ]
    },
    {
        time: "2021/2/22",
        version: "2.1.0",
        changes: [
            "增加魈的专属目标函数",
            "增加圣遗物编辑功能",
            "修复若干bug",
            "优化一小部分的推荐算法"
        ]
    },
    {
        time: "2021/2/23",
        version: "2.1.1",
        changes: [
            "增加迪奥娜专属目标函数",
            "新增禁用结果按钮",
            "添加算法说明",
            "修复先选突破后再选突破前等级时无法正确计算的bug",
        ]
    },
    {
        time: "2021/2/24",
        version: "2.2.0",
        changes: [
            "增加护摩之杖、千岩古剑、千岩长枪（粗略计算这些武器都很离谱）",
            "增加诺艾尔专属目标函数",
            "新增“50%暴击”目标函数",
            "新增“全部启用”按钮",
            "新增json增量导入",
            "修复“狼的末路”无法正确计算的bug",
        ]
    },
    {
        time: "2021/2/25",
        version: "2.3.0",
        changes: [
            "重构目标函数的逻辑，使之有能力计算动态buff收益",
            "角色面板可以查看详细属性（详细暴击率、伤害加成）" ,
            "重写甘雨目标函数，使之能够适应阿莫斯之弓、天赋、冰4套的动态加成",
            "重构通用目标函数，可以适应不同角色使用不同的属性",
        ]
    },
    {
        time: "2021/2/26",
        version: "2.3.1",
        changes: [
            "增加了技能等级的支持，某些角色（如诺艾尔、钟离等）计算更精确",
            "修复黎明神剑无法正确计算的bug",
            "新增某些武器的特殊参数",
            "修复甘雨目标函数的bug",
            "新增阿贝多、芭芭拉的目标函数",
        ]
    },
    {
        time: "2021/2/27",
        version: "2.3.2",
        changes: [
            "新增融化反应目标函数",
            "增加胡桃（非正式服数据）",
        ]
    },
    {
        time: "2021/3/2",
        version: "2.4.0",
        changes: [
            "重构目标函数逻辑，目标函数可以进行更精细的配置",
            "大幅优化刻晴目标函数",
        ]
    },
    {
        time: "2021/3/3",
        version: "2.4.1",
        changes: [
            "优化胡桃目标函数，现在可以自定义蒸发、融化频率，重击比例等",
        ]
    },
    {
        time: "2021/3/4",
        version: "2.4.2",
        changes: [
            "增加“温迪-风神之诗”目标函数",
            "增加“期望伤害-充能”目标函数"
        ]
    },
    {
        time: "2021/3/6",
        version: "2.4.3",
        changes: [
            "新增随机副词条功能，方便计算某个角色适合什么圣遗物"
        ]
    },
    {
        time: "2021/3/7",
        version: "2.4.4",
        changes: [
            "新增“元素反应-增幅反应-期望伤害”目标函数",
            "支持限定圣遗物的主词条",
        ]
    },
    {
        time: "2021/3/10",
        version: "3.0.0",
        changes: [
            "新增计算：圣遗物潜力值",
            "圣遗物支持等级和品质",
        ]
    },
    {
        time: "2021/3/11",
        version: "3.0.1",
        changes: [
            "新增“固定暴击率-期望伤害”目标函数",
        ]
    },
    {
        time: "2021/3/12",
        version: "3.0.2",
        changes: [
            "修复阿贝多2命情况下无法计算的问题",
        ]
    }
]