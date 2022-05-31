
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4825(_: S4, _: S7, _: S8) {}
        
        fn test4825() { foo4825(S1, S2, S3, S4, S6, S8); }
    