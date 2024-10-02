use yew::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Properties)]
pub struct ResultTableProps {
    pub data: Vec<PickHistory>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PickHistory {
    pub time: String,
    #[serde(rename = "pickGroup")]
    pub pick_group: String,
}

#[function_component(ResultTable)]
pub fn result_table(props: &ResultTableProps) -> Html {
    let format_date = |time: &str| {
        DateTime::parse_from_rfc3339(time)
            .map(|dt| dt.with_timezone(&Utc).format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|_| time.to_string())
    };

    if props.data.is_empty() {
        html! {}
    } else {
        html! {
            <div class="table-container">
                <p class="history-title">{"历史记录"}</p>
                <table>
                    <thead>
                        <tr>
                            <th>{"生成时间"}</th>
                            <th>{"选择英雄组合"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {props.data.iter().map(|row_data| {
                            html! {
                                <tr>
                                    <td class="table-divider">{format_date(&row_data.time)}</td>
                                    <td class="table-divider">{&row_data.pick_group}</td>
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        }
    }
}