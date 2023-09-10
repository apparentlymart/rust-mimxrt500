#![no_std]

use mimxrt500_bootstub::{bootstub_builtin, fcb};
use mimxrt595_evk::bootstub::FCB;

bootstub_builtin!();
fcb!(FCB);
