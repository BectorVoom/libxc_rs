//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1345/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1345<F: Float>(t25: F, t265: F, t394: F, t83543: F, t2116: F, t2250: F, t24555: F, t40: F, t607: F, t7274: F, t82334: F, t9258: F, t1240: F, t3630: F, t11588: F, t2127: F, t221: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t85617 = piecewise3::<f64>(t395, F::new(0.0), t83543);
    let t85627 = piecewise3::<f64>(t115, t82334, t85617 * t40 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t24555 * t607 + F::new(3.0) / F::new(2.0) * t7274 * t2250 + t2116 * t9258 / F::new(2.0));
    let t85628 = t1240 * t3630;
    let t85639 = t2127 * t221 * t11588;
    (t85627, t85628, t85639)
}
