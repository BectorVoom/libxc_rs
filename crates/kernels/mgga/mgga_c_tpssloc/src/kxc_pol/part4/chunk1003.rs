//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1003/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1003<F: Float>(t5544: F, t845: F, t776: F, t16662: F, t824: F, t1504: F, t1506: F, t16723: F, t16729: F, t16737: F, t16740: F, t228: F, t230: F, t4219: F, t4225: F, t4227: F, t4230: F, t5601: F, t5605: F, t5608: F, t822: F, t825: F) -> F {
    let t16745 = t845 * t5544;
    let t16746 = t16745 * t776;
    let t16749 = t824 * t16662;
    let t16752 = F::new(6.0) * t1504 * t4230 + F::new(6.0) * t1506 * t4219 - t16723 * t230 - F::new(24.0) * t16729 * t4227 + F::new(60.0) * t16737 * t4225 - F::new(24.0) * t16740 * t4225 - F::new(12.0) * t16746 * t4225 + F::new(3.0) * t16749 * t228 + F::new(3.0) * t5601 * t825 - F::new(12.0) * t5605 * t822 + F::new(3.0) * t5608 * t822;
    t16752
}
