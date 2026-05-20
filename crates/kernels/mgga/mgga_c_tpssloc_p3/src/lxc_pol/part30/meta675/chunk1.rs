//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2105/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105<F: Float>(t19440: F, t71: F, t33: F, t55880: F, t5441: F, t645: F, t72: F, t5389: F, t641: F, t12568: F, t1410: F, t1860: F, t1863: F, t1865: F, t22544: F, t26084: F, t26090: F, t27950: F, t27953: F, t27956: F, t27957: F, t27961: F, t6490: F, t6495: F, t6505: F, t83741: F, t83827: F) -> F {
    let t96379 = t71 * t19440;
    let t96383 = t55880 * t33;
    let t96393 = t72 * t5441 * t645;
    let t96403 = t72 * t641 * t5389;
    let t96406 = t12568 * t1410;
    let t96409 = -t1860 * t6505 * t27956 / F::new(6.0) - t1860 * t1863 * t96379 / F::new(6.0) - t96383 * t1865 / F::new(6.0) + t6495 * t27950 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t26084 * t26090 + F::new(2.0) / F::new(3.0) * t6495 * t27953 + F::new(5.0) / F::new(6.0) * t6490 * t96393 + t6495 * t27957 / F::new(3.0) - F::new(5.0) * t83827 * t27961 - F::new(5.0) * t83741 * t27961 - F::new(5.0) * t22544 * t96403 + F::new(2.0) / F::new(3.0) * t96406 * t1865;
    t96409
}
