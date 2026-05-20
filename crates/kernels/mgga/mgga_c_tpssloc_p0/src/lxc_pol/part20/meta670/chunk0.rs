//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2517/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517<F: Float>(t136: F, t3297: F, t50964: F, t2403: F, t4772: F, t14792: F, t699: F, t1113: F, t50929: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F) -> (F, F, F, F, F) {
    let t51049 = t136 * t3297 * t50964;
    let t51051 = t2403 * t4772;
    let t51053 = t699 * t14792;
    let t51056 = t136 * t1113 * t50929;
    let t51058 = F::new(4.0) / F::new(9.0) * t50826;
    let t51073 = F::new(8.0) / F::new(27.0) * t50919;
    let t51078 = t51058 - t50828 / F::new(3.0) + t50832 / F::new(3.0) - F::new(28.0) / F::new(81.0) * t50834 + F::new(2.0) / F::new(9.0) * t43727 - F::new(2.0) / F::new(3.0) * t43729 - F::new(8.0) / F::new(27.0) * t43748 - F::new(10.0) / F::new(81.0) * t43750 - F::new(2.0) / F::new(9.0) * t50897 - F::new(8.0) * t50900 - F::new(4.0) / F::new(3.0) * t50903 - F::new(2.0) / F::new(3.0) * t50905 - F::new(2.0) * t50907 + F::new(10.0) / F::new(9.0) * t50912 + F::new(40.0) / F::new(9.0) * t50917 - t51073 - F::new(10.0) / F::new(27.0) * t50921 - F::new(80.0) / F::new(81.0) * t50926 + F::new(2.0) * t50931 + F::new(2.0) * t50934;
    (t51049, t51051, t51053, t51056, t51078)
}
