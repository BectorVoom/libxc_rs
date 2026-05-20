//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2323/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2323<F: Float>(t46447: F, t5499: F, t58972: F, t12939: F, t17635: F, t4195: F, t20217: F, t707: F, t751: F, t1462: F, t58976: F, t39549: F, t39563: F, t39585: F, t39590: F, t40801: F, t40803: F, t67216: F, t67217: F, t67226: F, t67228: F, t67231: F, t67244: F) -> (F, F, F, F, F, F) {
    let t67457 = F::new(36.0) * t46447 * t5499;
    let t67458 = F::cast_from(0.32530743900905219526e-1_f64) * t58972;
    let t67461 = F::new(72.0) * t12939 * t4195 * t17635;
    let t67463 = t707 * t751 * t20217;
    let t67464 = F::new(4.0) * t67463;
    let t67466 = F::new(12.0) * t58976 * t1462;
    let t67467 = t40801 - t40803 - t67216 + t67217 + t39549 + t39563 + t67226 + t67228 + t67231 + t67244 + t67457 + t67458 + t67461 + t67464 + t67466 - t39585 + t39590;
    (t67457, t67458, t67461, t67464, t67466, t67467)
}
