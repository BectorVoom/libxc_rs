//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1445/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445<F: Float>(t50846: F, t63888: F, t63893: F, t63911: F, t71335: F, t71337: F, t71408: F, t77959: F, t77963: F, t77967: F, t78084: F, t44466: F, t71470: F, t71472: F, t71474: F, t77971: F, t77975: F, t77979: F, t77983: F, t78087: F, t78090: F, t78093: F, t78100: F) -> (F, F) {
    let t78596 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63888 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t63893 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t71335 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t71337 + F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t50846 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t77959 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t77963 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t63911 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t71408 + t77967 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78084;
    let t78607 = F::cast_from(2.0_f64) * t78087 - t77971 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78090 - F::cast_from(6.0_f64) * t78093 + F::cast_from(2.0_f64) * t77975 - F::cast_from(4.0_f64) * t77979 - t77983 / F::cast_from(6.0_f64) - t44466 + F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t71470 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t78100 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t71472 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t71474;
    (t78596, t78607)
}
