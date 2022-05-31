
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10242(_: S2, _: S1, _: S6, _: S1, _: S2) {}
        
        fn test10242() { foo10242(S1, S2, S3, S4, S6, S7, S8); }
    