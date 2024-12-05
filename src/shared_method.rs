use build::model::{EnumErrorCode, EnumRole};
use eyre::*;
use eyre::{ensure, ContextCompat};
use endpoint_libs::libs::toolbox::{CustomError, RequestContext};
use num_traits::FromPrimitive;

pub fn ensure_user_role(ctx: RequestContext, role: EnumRole) -> Result<()> {
    let ctx_role = EnumRole::from_u32(ctx.role).context("Invalid role")?;
    ensure!(
        ctx_role >= role,
        CustomError::new(
            EnumErrorCode::InvalidRole,
            format!("Requires {} Actual {}", role, ctx_role)
        )
    );
    Ok(())
}
