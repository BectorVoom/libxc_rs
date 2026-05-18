//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1401/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1401<F: Float>(t33429: F, t6547: F, t7841: F, t857: F, t22986: F, t23270: F, t776: F, t114900: F, t118859: F, t118871: F, t118874: F, t2054: F, t23278: F, t25168: F, t25188: F, t25232: F, t26700: F, t26728: F, t2713: F, t33433: F, t6663: F, t7107: F, t7830: F, t7842: F, t86988: F) -> F {
    let t121629 = t6547 * t33429;
    let t121634 = t857 * t7841;
    let t121637 = t22986 * t23270 * t121634 * t776;
    let t121643 = -F::new(6.0) * t25168 * t26728 * t25232 - t25188 * t7107 - t23278 * t7842 - F::new(0.19190897446562641759e-1) * t121629 + t118859 + F::new(2.0) * t23278 * t7830 - t86988 * t2054 - t118871 + F::new(0.16449340668482264365e-1) * t121637 + F::new(2.0) * t2713 * t33433 - t118874 + F::new(0.38381794893125283518e-1) * t114900 - t26700 * t6663;
    t121643
}
