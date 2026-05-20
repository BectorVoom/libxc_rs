//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2329/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329<F: Float>(t1506: F, t16723: F, t16729: F, t16737: F, t16740: F, t16746: F, t20835: F, t225: F, t230: F, t232: F, t4219: F, t4227: F, t4230: F, t5601: F, t5605: F, t5608: F, t67448: F, t67449: F, t67451: F, t67452: F, t67455: F, t67467: F, t67491: F, t67509: F, t67566: F, t68: F, t825: F) -> F {
    let t67568 = (-(t67448 + t67449 + t67451 + t67452 + t67455 + t67467 + t67491 + t67509) * t225 * t230 + F::new(3.0) * t20835 * t825 + F::new(9.0) * t16723 * t1506 - F::new(36.0) * t5601 * t68 * t4227 + F::new(9.0) * t5601 * t4230 - F::new(36.0) * t4219 * t5605 + F::new(180.0) * t16729 * t16737 - F::new(72.0) * t16729 * t16740 + F::new(9.0) * t4219 * t5608 - F::new(36.0) * t16729 * t16746 + t67566) * t232;
    t67568
}
