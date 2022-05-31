
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10927(_: S3, _: S2, _: S3, _: S2, _: S4, _: S1) {}
        
        fn test10927() { foo10927(S4, S6, S4, S2, S1, S3, S5); }
    