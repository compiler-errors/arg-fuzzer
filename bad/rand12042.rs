
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12042(_: S2, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test12042() { foo12042(S3, S5, S6, S1, S4, S0, S2); }
    