//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1819/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1819<F: Float>(t3109: F, t4630: F, t4650: F, t884: F, t3071: F, t10436: F, t10441: F, t10449: F, t10455: F, t10460: F, t10490: F, t10496: F, t10504: F, t10511: F, t10517: F, t10863: F, t10866: F, t10871: F, t1618: F, t1622: F, t3048: F, t3070: F, t4636: F) -> (F, F, F) {
    let t14059 = t3109 * t4630 / F::new(432.0);
    let t14068 = t4650 * t884;
    let t14069 = t3071 * t14068;
    let t14074 = -t10436 / F::new(6912.0) - t10441 / F::new(432.0) + F::new(19.0) / F::new(2592.0) * t10449 + t10455 / F::new(6912.0) + F::new(5.0) / F::new(20736.0) * t10460 + F::new(19.0) / F::new(1728.0) * t10517 * t1618 - t14059 - t10863 * t1622 / F::new(432.0) - t3048 * t4636 / F::new(432.0) - t10490 / F::new(3456.0) - t10496 / F::new(432.0) + t10504 / F::new(2304.0) - t10511 / F::new(6912.0) + t3070 * t14069 / F::new(2304.0) + t10866 / F::new(3456.0) - t10871 / F::new(10368.0);
    (t14068, t14069, t14074)
}
