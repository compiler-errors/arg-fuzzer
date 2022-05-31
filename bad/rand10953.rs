
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10953(_: S1, _: S3, _: S4, _: S7) {}
        
        fn test10953() { foo10953(S4, S1, S4, S3, S2); }
    