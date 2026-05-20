//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1265/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1265<F: Float>(t81742: F, t6612: F, t812: F, t836: F, t2690: F, t6619: F, t131: F, t23121: F, t9537: F, t236: F, t81613: F, t22822: F, t281: F, t6589: F) -> (F, F, F, F, F, F) {
    let t81743 = F::cast_from(0.43737152435318756759e-3_f64) * t81742;
    let t81749 = t812 * t6612 * t836;
    let t81763 = t812 * t6619 * t2690;
    let t81782 = t23121 * t131 * t9537;
    let t81783 = t81613 * t236;
    let t81788 = t22822 * t6589 * t281;
    (t81743, t81749, t81763, t81782, t81783, t81788)
}
