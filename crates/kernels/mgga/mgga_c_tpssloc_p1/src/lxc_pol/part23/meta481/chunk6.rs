//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1445/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445<F: Float>(t50846: F, t63888: F, t63893: F, t63911: F, t71335: F, t71337: F, t71408: F, t77959: F, t77963: F, t77967: F, t78084: F, t44466: F, t71470: F, t71472: F, t71474: F, t77971: F, t77975: F, t77979: F, t77983: F, t78087: F, t78090: F, t78093: F, t78100: F) -> (F, F) {
    let t78596 = F::new(10.0) / F::new(27.0) * t63888 - F::new(20.0) / F::new(9.0) * t63893 - F::new(4.0) / F::new(9.0) * t71335 + F::new(8.0) / F::new(3.0) * t71337 + F::new(160.0) / F::new(81.0) * t50846 - F::new(8.0) / F::new(9.0) * t77959 + F::new(14.0) / F::new(81.0) * t77963 - F::new(10.0) / F::new(9.0) * t63911 + F::new(4.0) / F::new(9.0) * t71408 + t77967 / F::new(6.0) + F::new(2.0) / F::new(9.0) * t78084;
    let t78607 = F::new(2.0) * t78087 - t77971 - F::new(4.0) / F::new(3.0) * t78090 - F::new(6.0) * t78093 + F::new(2.0) * t77975 - F::new(4.0) * t77979 - t77983 / F::new(6.0) - t44466 + F::new(16.0) / F::new(81.0) * t71470 - F::new(4.0) / F::new(9.0) * t78100 - F::new(8.0) / F::new(9.0) * t71472 + F::new(8.0) / F::new(3.0) * t71474;
    (t78596, t78607)
}
