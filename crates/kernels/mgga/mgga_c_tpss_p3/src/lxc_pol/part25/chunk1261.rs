//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1261/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1261<F: Float>(t1639: F, t520: F, t6419: F, t5745: F, t1838: F, t5407: F, t21834: F, t1773: F, t21804: F, t522: F, t1657: F, t1772: F, t1842: F, t18496: F, t19509: F, t20157: F, t21061: F, t21805: F, t21820: F, t21823: F, t21827: F, t21831: F, t21836: F, t538: F, t5433: F, t5449: F, t5739: F, t5921: F, t6260: F, t6425: F, t6430: F, t6433: F) -> (F, F, F, F, F) {
    let t21840 = t6419 * t1639 * t520;
    let t21841 = t5745 * t21840;
    let t21846 = t5745 * t1838 * t5407 * t520;
    let t21849 = t5745 * t21834 * t520;
    let t21852 = t1773 * t522 * t21804;
    let t21854 = -F::cast_from(2.0_f64) * t1657 * t20157 - t1772 * t21852 - t1842 * t21061 - F::cast_from(4.0_f64) * t18496 * t21823 + F::cast_from(4.0_f64) * t19509 * t6425 + F::cast_from(2.0_f64) * t19509 * t6430 + t21805 * t538 - F::cast_from(6.0_f64) * t21820 * t5739 + F::cast_from(4.0_f64) * t21827 * t5739 + F::cast_from(2.0_f64) * t21831 * t5739 - F::cast_from(2.0_f64) * t21836 * t5739 + F::cast_from(2.0_f64) * t21841 * t5739 + t21846 * t5739 + t21849 * t5739 + F::cast_from(2.0_f64) * t5433 * t5921 - t5449 * t5921 - F::cast_from(2.0_f64) * t6260 * t6433;
    (t21841, t21846, t21849, t21852, t21854)
}
