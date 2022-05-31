
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11097(_: S1, _: S2, _: S4, _: S5) {}
        
        fn test11097() { foo11097(S1, S7, S4, S1, S2, S3); }
    