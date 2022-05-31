
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10284(_: S2, _: S4, _: S8) {}
        
        fn test10284() { foo10284(S2, S3, S4, S6); }
    