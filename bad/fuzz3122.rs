
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3122(_: S5, _: S7) {}
        
        fn test3122() { foo3122(S4, S5, S3, S2, S1); }
    