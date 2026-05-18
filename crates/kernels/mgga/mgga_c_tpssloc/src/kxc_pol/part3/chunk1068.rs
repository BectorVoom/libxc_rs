//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1068/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1068<F: Float>(t13823: F, t973: F, t2970: F, t4522: F, t6733: F, t884: F, t4531: F, t10254: F, t3961: F, t2988: F, t10236: F, t10235: F) -> (F, F, F, F, F) {
    let t13825 = F::new(0.55555555555555555554e-3) * t973 * t13823;
    let t13828 = t2970 * t4522;
    let t13830 = F::new(0.18518518518518518518e-3) * t973 * t13828;
    let t13831 = t6733 * t884;
    let t13832 = t4531 * t13831;
    let t13835 = t10254 * t3961;
    let t13836 = t2988 * t13835;
    let t13839 = t10236 * t3961;
    let t13840 = t10235 * t13839;
    (t13825, t13830, t13832, t13836, t13840)
}
