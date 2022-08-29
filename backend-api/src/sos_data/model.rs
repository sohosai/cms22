use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectCategory {
    #[serde(rename = "オンライン一般企画")]
    GeneralOnline,
    #[serde(rename = "対面一般企画")]
    GeneralPhysical,
    #[serde(rename = "オンラインステージ企画")]
    StageOnline,
    #[serde(rename = "対面ステージ企画")]
    StagePhysical,
    #[serde(rename = "調理企画")]
    CookingPhysical,
    #[serde(rename = "飲食物取扱い企画")]
    FoodPhysical,
}

//FIXME:  Use a suitable type for date and bool.(It occurd parse error so using string for now)
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectRecord {
    #[serde(rename = "企画ID")]
    pub project_id: String,
    #[serde(rename = "企画番号")]
    pub project_code: String,
    #[serde(rename = "企画登録日時")]
    pub registration_date: String,
    #[serde(rename = "企画最終更新日時")]
    pub last_update_at: String,
    #[serde(rename = "責任者ユーザーID")]
    pub owner_user_id: String,
    #[serde(rename = "責任者 名")]
    pub owner_given_name: String,
    #[serde(rename = "責任者 姓")]
    pub owner_family_name: String,
    #[serde(rename = "責任者 名(かな)")]
    pub owner_given_name_kana: String,
    #[serde(rename = "責任者 姓(かな)")]
    pub owner_family_name_kana: String,
    #[serde(rename = "副責任者ユーザーID")]
    pub subowner_user_id: String,
    #[serde(rename = "副責任者 名")]
    pub subowner_given_name: String,
    #[serde(rename = "副責任者 姓")]
    pub subowner_family_name: String,
    #[serde(rename = "副責任者 名(かな)")]
    pub subowner_given_name_kana: String,
    #[serde(rename = "副責任者 姓(かな)")]
    pub subowner_family_name_kana: String,
    #[serde(rename = "企画名")]
    pub project_name: String,
    #[serde(rename = "企画名(かな)")]
    pub project_name_kana: String,
    #[serde(rename = "団体名")]
    pub organization_name: String,
    #[serde(rename = "団体名(かな)")]
    pub organization_name_kana: String,
    #[serde(rename = "説明文")]
    pub description: String,
    #[serde(rename = "企画区分")]
    pub project_category: ProjectCategory,
    #[serde(rename = "学術参加枠")]
    pub is_academic: String,
    #[serde(rename = "芸術祭参加枠")]
    pub is_art: String,
    #[serde(rename = "委員会企画")]
    pub is_committee: String,
    #[serde(rename = "屋外企画")]
    pub is_outdoor: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Role {
    #[serde(rename = "一般")]
    Participant,
    #[serde(rename = "実委人(管理者)")]
    CommitteeOperator,
    #[serde(rename = "実委人")]
    Committee,
    #[serde(rename = "SOS管理者")]
    Administrator,
}

impl Role {
    pub fn is_committee(&self) -> bool {
        match self {
            Role::CommitteeOperator | Role::Committee | Self::Administrator => true,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum UserCategory {
    #[serde(rename = "学部生")]
    Undergraduate,
    #[serde(rename = "教職員")]
    Staff,
    #[serde(rename = "院生")]
    Graduate,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserRecord {
    #[serde(rename = "ユーザーID")]
    pub user_id: String,
    #[serde(rename = "登録日時")]
    pub registration_date: String, // FIXME:  Use a suitable type for date.
    #[serde(rename = "名")]
    pub given_name: String,
    #[serde(rename = "姓")]
    pub family_name: String,
    #[serde(rename = "名(かな)")]
    pub given_name_kana: String,
    #[serde(rename = "姓(かな)")]
    pub family_name_kana: String,
    #[serde(rename = "メールアドレス")]
    pub email: String,
    #[serde(rename = "電話番号")]
    pub phone_number: String,
    #[serde(rename = "SOS権限")]
    pub role: Role,
    #[serde(rename = "区分")]
    pub category: UserCategory,
}

impl Into<crate::model::UserProfile> for UserRecord {
    fn into(self) -> crate::model::UserProfile {
        crate::model::UserProfile {
            email: self.email,
            name: format!(
                "{} {}",
                self.family_name.to_string(),
                self.given_name.to_string()
            ),
            is_committee: self.role.is_committee(),
        }
    }
}
