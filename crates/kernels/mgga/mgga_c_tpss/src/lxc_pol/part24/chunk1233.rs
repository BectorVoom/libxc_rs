//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1233/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1233<F: Float>(t33: F, t259: F, t479: F, t21476: F, t1289: F, t1749: F, t21523: F, t4579: F, t57: F, t6222: F, t21484: F, t118: F, t1322: F, t1339: F, t1663: F, t1753: F, t1757: F, t19462: F, t21198: F, t21202: F, t21205: F, t21208: F, t21213: F, t21234: F, t21238: F, t21240: F, t21241: F, t21247: F, t21254: F, t3493: F, t4631: F, t4641: F, t4675: F, t544: F, t5463: F, t5514: F, t6117: F, t6228: F, t6239: F, t626: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t21524 = piecewise3(t480, 0.0, t21476);
    let t21531 = piecewise3(t386, t21523, t21524 * t57 / 2.0 - t6222 * t1289 - t1749 * t4579 / 2.0);
    let t21532 = t21484 + t21531;
    let t21534 = -t118 * t21532 - 2.0 * t1322 * t6228 - 4.0 * t1339 * t19462 + 2.0 * t1663 * t6239 - t1753 * t4631 + t1757 * t5463 - 2.0 * t21208 * t626 + t21234 * t544 - 4.0 * t21241 * t626 - 4.0 * t3493 * t6117 - 4.0 * t4641 * t5514 - 2.0 * t4675 * t5514 - t21198 - t21202 - t21205 - t21213 - t21238 - t21240 + t21247 + t21254;
    (t21524, t21532, t21534)
}
