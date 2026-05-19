//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1004/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1004<F: Float>(t25: F, t265: F, t394: F, t128193: F, t128093: F, t128134: F, t1409: F, t33513: F, t40: F, t5398: F, t8580: F, t100688: F, t101226: F, t101840: F, t126992: F, t127017: F, t127030: F, t128076: F, t128080: F, t128101: F, t1877: F, t24191: F, t2522: F, t25927: F, t26744: F, t26756: F, t28: F, t28764: F, t28778: F, t33476: F, t33483: F, t33537: F, t33539: F, t4314: F, t5966: F, t7114: F, t8566: F, t8586: F, t89992: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t128194 = piecewise3::<F>(t395, F::new(0.0), t128193);
    let t128201 = piecewise3::<F>(t115, t128093 + t128134, t128194 * t40 / F::new(2.0) + t33513 * t1409 + t8580 * t5398 / F::new(2.0));
    let t128239 = F::new(2.0) * t26756 * t100688 * t33483 + F::new(2.0) * t26756 * t127030 + t26756 * t25927 * t128101 + t1877 * t8566 * t5966 / F::new(2.0) + t1877 * t128076 * t28 / F::new(2.0) - F::new(3.0) * t24191 * t89992 * t33476 + F::new(6.0) * t24191 * t25927 * t128080 + F::new(3.0) / F::new(2.0) * t2522 * t8566 * t28778 + F::new(3.0) * t4314 * t8566 * t28764 - t1877 * t7114 * t127017 - t1877 * t26744 * t33539 + F::new(2.0) * t101840 * t33537 - t1877 * t7114 * t126992 / F::new(2.0) - t1877 * t101226 * t8586 / F::new(2.0);
    (t128201, t128239)
}
