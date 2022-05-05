// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! The dry-run command.

use crate::{error::Error, prelude::*, DryRunConfig};

pub(crate) async fn run<M>(client: SubxtClient, cfg: DryRunConfig) -> Result<(), Error>
where
	M: MinerConfig<AccountId = AccountId, MaxVotesPerVoter = crate::chains::MinerMaxVotesPerVoter>
		+ 'static,
	<M as MinerConfig>::Solution: Send + Sync,
{
	todo!();
}
