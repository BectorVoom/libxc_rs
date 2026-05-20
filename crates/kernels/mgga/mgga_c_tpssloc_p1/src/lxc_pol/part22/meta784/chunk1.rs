//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2692/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692<F: Float>(t12250: F, t6414: F, t1824: F, t6434: F, t1336: F, t1352: F, t16047: F, t1825: F, t19654: F, t19657: F, t19744: F, t19748: F, t19815: F, t20490: F, t20568: F, t20622: F, t3777: F, t3901: F, t40492: F, t5250: F, t5287: F, t5334: F, t5335: F, t5344: F, t5349: F, t57618: F, t74174: F, t74941: F) -> (F, F) {
    let t75008 = t12250 * t6414;
    let t75026 = t6434 * t1824;
    let t75053 = -F::new(3.0) * t1336 * t19657 * t5287 - F::new(6.0) * t1336 * t20490 * t40492 - t1336 * t20568 * t3901 - F::new(3.0) * t1352 * t5344 * t75026 - F::new(18.0) * t16047 * t19744 * t74941 - F::new(3.0) * t1825 * t5344 * t57618 + F::new(6.0) * t5250 * t5334 * t75026 + F::new(6.0) * t5334 * t5335 * t74174 + F::new(18.0) * t19654 * t19748 - F::new(3.0) * t19815 * t5349 - F::new(6.0) * t20622 * t3777;
    (t75008, t75053)
}
