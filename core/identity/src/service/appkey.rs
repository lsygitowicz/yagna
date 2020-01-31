use uuid::Uuid;

use ya_persistence::executor::DbExecutor;

use crate::dao::AppKeyDao;

pub async fn activate(db: &DbExecutor) -> anyhow::Result<()> {
    use ya_core_model::appkey as model;
    use ya_service_bus::{typed as bus, RpcEndpoint};

    let dbx = db.clone();

    // Create a new application key entry
    let _ = bus::bind_private(&model::SERVICE_ID, move |create: model::Create| {
        let key = Uuid::new_v4().to_simple().to_string();
        let db = dbx.clone();
        async move {
            Ok(db
                .as_dao::<AppKeyDao>()
                .create(key.clone(), create.name, create.role, create.identity)
                .await
                .map_err(|e| model::Error::internal(e))
                .map(|_| key)?)
        }
    });

    let dbx = db.clone();
    // Retrieve an application key entry based on the key itself
    let _ = bus::bind(&model::BUS_ID, move |get: model::Get| {
        let db = dbx.clone();
        async move {
            let (appkey, role) = if let Some(key) = get.key {
                db.as_dao::<AppKeyDao>()
                    .get(key)
                    .await
                    .map_err(Into::into)?
            } else {
                use ya_core_model::identity as idm;
                let identity: idm::IdentityInfo = bus::service(idm::BUS_ID)
                    .send(idm::Get::ByDefault)
                    .await
                    .unwrap() //FIXME
                    .unwrap() //FIXME
                    .unwrap(); //FIXME
                db.as_dao::<AppKeyDao>()
                    .get_for_id(identity.node_id.to_string())
                    .await
                    .map_err(Into::into)?
            };
            Ok(model::AppKey {
                name: appkey.name,
                key: appkey.key,
                role: role.name,
                identity: appkey.identity_id,
                created_date: appkey.created_date,
            })
        }
    });

    let dbx = db.clone();
    let _ = bus::bind(model::BUS_ID, move |list: model::List| {
        let db = dbx.clone();

        async move {
            let result = db
                .as_dao::<AppKeyDao>()
                .list(list.identity, list.page, list.per_page)
                .await
                .map_err(Into::into)?;

            let keys = result
                .0
                .into_iter()
                .map(|(app_key, role)| model::AppKey {
                    name: app_key.name,
                    key: app_key.key,
                    role: role.name,
                    identity: app_key.identity_id,
                    created_date: app_key.created_date,
                })
                .collect();

            Ok((keys, result.1))
        }
    });

    let dbx = db.clone();
    let _ = bus::bind(&model::BUS_ID, move |rm: model::Remove| {
        let db = dbx.clone();
        async move {
            db.as_dao::<AppKeyDao>()
                .remove(rm.name, rm.identity)
                .await
                .map_err(Into::into)?;
            Ok(())
        }
    });

    Ok(())
}
