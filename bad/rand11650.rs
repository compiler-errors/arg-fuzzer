
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11650(_: S3, _: S6) {}
        
        fn test11650() { foo11650(S8, S3, S1); }
    