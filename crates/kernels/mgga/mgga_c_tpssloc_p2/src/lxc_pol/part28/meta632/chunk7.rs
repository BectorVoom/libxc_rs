//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1996/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1996<F: Float>(t870: F, t92989: F, t10143: F, t7844: F, t1877: F, t2057: F, t22964: F, t23296: F, t24191: F, t25: F, t2522: F, t25385: F, t26563: F, t26740: F, t26756: F, t6542: F, t7110: F, t7114: F, t7845: F, t86718: F, t86722: F, t86798: F, t86821: F, t87984: F, t87998: F, t92356: F, t92359: F, t92362: F, t92364: F) -> (F, F, F) {
    let t92990 = t92989 * t870;
    let t93000 = t7844 * t10143;
    let t93005 = -F::new(3.0) * t26756 * t86718 - t1877 * t7114 * t87984 / F::new(2.0) + t92356 - t92359 + t92362 - t92364 + F::new(3.0) * t2522 * t26740 * t6542 - F::new(6.0) * t26563 * t86798 - F::new(3.0) * t24191 * t87998 + F::new(3.0) * t2522 * t7845 * t22964 + t1877 * t92990 * t25 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t86821 + F::new(3.0) * t2522 * t7110 * t25385 + t1877 * t93000 * t23296 - F::new(3.0) * t24191 * t86722;
    (t92990, t93000, t93005)
}
