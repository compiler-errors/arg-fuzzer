
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2329(_: S1, _: S3, _: S6) {}
        
        fn test2329() { foo2329(S4, S7, S6, S1, S5, S2, S4, S7); }
    