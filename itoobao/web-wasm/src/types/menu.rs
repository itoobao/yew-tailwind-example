#[derive(Clone, Debug)]
pub struct Menu {
    pub id: u32,
    pub name: String,
    pub items: Vec<Menu>,
}

impl Menu {
    pub fn menu_list() -> Vec<Self> {
        vec![
            Self {
                id: 1,
                name: "Dashbord".to_string(),
                items: vec![
                    Self {
                        id: 2,
                        name: "用户列表".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 3,
                        name: "添加用户".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 4,
                        name: "黑名单列表".to_string(),
                        items: vec![],
                    },
                ],
            },
            Self {
                id: 5,
                name: "触达管理".to_string(),
                items: vec![
                    Self {
                        id: 6,
                        name: "渠道管理".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 7,
                        name: "用户分层".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 8,
                        name: "群发模版".to_string(),
                        items: vec![],
                    },
                ],
            },
            Self {
                id: 9,
                name: "生命周期管理".to_string(),
                items: vec![
                    Self {
                        id: 10,
                        name: "生命周期".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 11,
                        name: "分组管理".to_string(),
                        items: vec![],
                    },
                    Self {
                        id: 12,
                        name: "数据统计".to_string(),
                        items: vec![],
                    },
                ],
            },
        ]
    }
}
