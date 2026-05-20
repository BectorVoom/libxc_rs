//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2452/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452<F: Float>(t1020: F, t1616: F, t248: F, t43216: F, t10489: F, t4644: F, t10898: F, t4630: F, t10882: F, t48569: F, t10463: F, t10493: F, t10517: F, t10886: F, t10891: F, t10937: F, t10972: F, t13762: F, t14080: F, t14099: F, t1618: F, t3098: F, t42496: F, t42653: F, t43186: F, t4579: F, t4652: F) -> F {
    let t50181 = t1020 * t248 * t43216 * t1616;
    let t50183 = t4644 * t10489;
    let t50189 = t10898 * t4630;
    let t50193 = t48569 * t10882;
    let t50207 = t10891 * t14099 / F::new(96.0) + t50181 / F::new(10368.0) - t50183 / F::new(1152.0) + t4644 * t10493 / F::new(768.0) + F::new(19.0) / F::new(576.0) * t42653 * t1618 - t50189 / F::new(144.0) + F::new(19.0) / F::new(576.0) * t10517 * t4652 + t50193 * t10886 / F::new(3072.0) + t4644 * t10463 / F::new(4608.0) + F::new(5.0) / F::new(5184.0) * t4644 * t10972 + t14080 * t3098 / F::new(144.0) - t42496 * t4579 / F::new(144.0) - t10937 * t13762 / F::new(144.0) + t43186 / F::new(1152.0);
    t50207
}
