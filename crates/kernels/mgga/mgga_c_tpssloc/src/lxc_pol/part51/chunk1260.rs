//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1260/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1260<F: Float>(t28: F, t265: F, t504: F, t121950: F, t121982: F, t122012: F, t122042: F, t122072: F, t1409: F, t31512: F, t33547: F, t3966: F, t52: F, t607: F, t8591: F, t113: F, t121958: F, t121231: F, t121233: F, t121234: F, t121237: F, t121240: F, t121253: F, t121254: F, t2039: F, t2075: F, t2314: F, t24983: F, t25958: F, t26098: F, t31734: F, t33350: F, t4034: F, t652: F, t7042: F, t7458: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t122075 = piecewise3(t505, 0.0, t121950);
    let t122082 = piecewise3(t401, t121982 + t122012 + t122042 + t122072, t122075 * t52 / 2.0 - t31512 * t1409 / 2.0 - t33547 * t607 / 2.0 - t8591 * t3966 / 2.0);
    let t122084 = t113 * (t121958 + t122082);
    let t122085 = -2.0 * t2039 * t25958 * t652 - t2075 * t26098 - 2.0 * t2314 * t33350 - 2.0 * t24983 * t7042 - 2.0 * t31734 * t7458 - 2.0 * t33350 * t4034 - t121231 - t121233 - t121234 - t121237 - t121240 - t121253 - t121254 - t122084;
    (t122085,)
}
