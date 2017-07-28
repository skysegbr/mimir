// Copyright (c) 2017 mimiron developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `mimiron` errors
error_chain!{
    foreign_links {
        Credentials(::rusoto_core::CredentialsError);
        DescribeDBInstances(::rusoto_rds::DescribeDBInstancesError);
        DescribeEventCategories(::rusoto_rds::DescribeEventCategoriesError);
        DescribeEventSubscriptions(::rusoto_rds::DescribeEventSubscriptionsError);
        DescribeEvents(::rusoto_rds::DescribeEventsError);
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        StartDBInstance(::rusoto_rds::StartDBInstanceError);
        StopDBInstance(::rusoto_rds::StopDBInstanceError);
        Term(::term::Error);
        Tls(::rusoto_core::TlsError);
    }

    errors {
        CreateTerm {
            description("")
            display("")
        }
        InvalidCommand {
            description("Invaid command!")
            display("Invalid command!")
        }
    }
}
