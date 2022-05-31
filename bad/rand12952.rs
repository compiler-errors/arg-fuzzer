
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12952(_: S4, _: S4, _: S6) {}
        
        fn test12952() { foo12952(S4, S5, S7, S5, S2, S4, S1); }
    