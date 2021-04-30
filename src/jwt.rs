use rutie::{RString, Hash, Exception, AnyException, Boolean, Symbol, VM};
use exogress_common::access_tokens;
use exogress_common::entities::AccessKeyId;

class!(Jwt);

methods!(
    Jwt,
    _rtself,

    fn generate_access_key_pair() -> Hash {
        let key_pair = access_tokens::generate_access_key_pair();

        let mut map = Hash::new();
        map.store(Symbol::new("secret_access_key"), RString::from(key_pair.secret_access_key));
        map.store(Symbol::new("public_key_pem"), RString::from(key_pair.public_key_pem));

        map
    }

    fn validate_jwt_token(
        public_key_pem: RString,
        access_key_id: RString,
        jwt_token: RString
    ) -> Boolean {
        let public_key_pem = public_key_pem.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let access_key_id = access_key_id
            .map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string()
            .parse::<AccessKeyId>()
            .map_err(|e| {
                let exception = AnyException::new("EntityException", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();
        let jwt_token = jwt_token.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        if let Err(_e) = access_tokens::validate_jwt_token(&public_key_pem, &access_key_id, &jwt_token) {
            Boolean::new(false)
        } else {
            Boolean::new(true)
        }
    }
);

