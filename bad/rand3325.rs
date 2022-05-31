
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3325(_: S2, _: S4, _: S6, _: S8) {}
        
        fn test3325() { foo3325(S3, S1, S5, S2, S7, S6, S7); }
    