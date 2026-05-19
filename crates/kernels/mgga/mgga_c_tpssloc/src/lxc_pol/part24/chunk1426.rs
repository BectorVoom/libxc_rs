//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1426/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1426<F: Float>(t5: F, t83732: F, t83766: F, t83812: F, t83849: F, t112: F, t531: F, t6995: F, t1983: F, t22596: F, t12012: F, t1390: F, t6878: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t83852 = piecewise3::<F>(t8, F::new(0.0), t83732 + t83766 + t83812 + t83849);
    let t83853 = t83852 * t112;
    let t83859 = t531 * t6995;
    let t83862 = F::new(18.0) * t1983 * t83859 * t22596;
    let t83863 = t1390 * t12012;
    let t83866 = F::new(3.0) * t1983 * t6878 * t83863;
    (t83853, t83862, t83866)
}
