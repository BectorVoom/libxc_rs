//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1044/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1044<F: Float>(t2976: F, t4205: F, t1089: F, t3009: F, t4198: F, t1542: F, t9172: F, t2975: F, t9176: F, t2973: F, t4180: F, t1082: F, t2998: F, t4206: F, t9384: F, t4101: F, t673: F) -> (F, F, F, F, F, F, F) {
    let t11819 = t4205 * t2976;
    let t11821 = 0.35089341735807877242e1 * t1089 * t11819;
    let t11823 = 0.23392894490538584828e1 * t3009 * t4198;
    let t11824 = t9172 * t1542;
    let t11825 = t9176 * t2975;
    let t11826 = t11824 * t11825;
    let t11828 = 0.10254018858216406658e4 * t1089 * t11826;
    let t11829 = t2973 * t4180;
    let t11830 = t11829 * t1082;
    let t11832 = 0.23392894490538584828e1 * t1089 * t11830;
    let t11833 = t2998 * t4180;
    let t11834 = t11833 * t4206;
    let t11836 = 0.34631718211362927518e2 * t1089 * t11834;
    let t11837 = t4205 * t9384;
    let t11839 = 0.17315859105681463759e2 * t1089 * t11837;
    let t11844 = t673 * t4101;
    (t11821, t11823, t11828, t11832, t11836, t11839, t11844)
}
