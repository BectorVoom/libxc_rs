//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1390/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1390<F: Float>(t10103: F, t858: F, t856: F, t68: F, t2719: F, t865: F, t2742: F, t2718: F, t10047: F, t10049: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t855: F, t866: F, t9520: F, t9585: F, t9587: F, t9590: F, t9593: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10104 = t858 * t10103;
    let t10108 = t856 * t856;
    let t10109 = F::new(1.0) / t10108;
    let t10110 = t68 * t10109;
    let t10111 = t2719 * t865;
    let t10112 = t10110 * t10111;
    let t10115 = t865 * t2742;
    let t10116 = t2718 * t10115;
    let t10121 = t10047 * t259 - F::new(3.0) * t10049 * t866 - t10104 * t855 - F::new(6.0) * t10112 * t855 + F::new(6.0) * t10116 * t855 + F::new(3.0) * t259 * t9520 + t259 * t9585 + F::new(3.0) * t259 * t9587 + F::new(6.0) * t2597 * t2720 - F::new(3.0) * t2597 * t2743 + F::new(6.0) * t2713 * t2720 - F::new(3.0) * t2713 * t2743 - F::new(3.0) * t866 * t9590 - F::new(6.0) * t866 * t9593;
    (t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121)
}
