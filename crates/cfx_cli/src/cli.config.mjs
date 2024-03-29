export default {
  name: "cf-design-pro",
  build: {
    namedExport: true,
    ignore: ["pro-button"],
    site: {
      publicPath: "/cf-design-pro/",
    },
    configureVite(config) {
      const { BUILD_TARGET } = process.env;

      if (BUILD_TARGET === "site") {
        // 修改文档站点构建配置
        config.server = {
          ...config.server,
          port: 3200,
        };
        config.build = {
          ...config.build,
          cssCodeSplit: false,
        };
      }

      return config;
    },
  },
  site: {
    title: "CF Design Pro",
    logo: "https://cf-wp-test.oss-cn-hangzhou.aliyuncs.com/W00000000109/customerCustomFile/ce7b1753-4438-4d3e-8186-42a39d7fdc28/1akacfvdzxw1636337483225.png",
    description: "尘锋-高级封装组件",
    hideSimulator: true,
    nav: [
      {
        title: "开发指南",
        items: [
          {
            path: "changelog",
            title: "更新日志",
          },
        ],
      },
      {
        title: "数据展示",
        items: [
          {
            path: "pro-table",
            title: "ProTable 高级表格",
          },
          {
            path: "pro-tooltip",
            title: "ProTooltip 高级文字提示",
          },
          {
            path: "pro-icon",
            title: "ProIcon 图标",
          }
        ],
      },
      {
        title: "反馈",
        items: [
          {
            path: "pro-drawer",
            title: "ProDrawer 高级抽屉",
          },
          {
            path: "pro-spin",
            title: "ProSpin 高级加载中",
          }
        ],
      },
    ],
  },
};
